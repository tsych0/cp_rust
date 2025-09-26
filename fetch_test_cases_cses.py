import subprocess
import requests
import zipfile
import io


# 1️⃣ Get problem URL from local cp-assist
def fetch_problem_url() -> str:
    try:
        resp = requests.get("http://localhost:27121/")
        resp.raise_for_status()
        problem = resp.json()
        url = problem.get("url")
        if not url:
            raise RuntimeError("Problem URL missing 😵")
        print(f"Got Problem url ✅: {url}")
        return url
    except Exception as e:
        raise RuntimeError(f"Cannot fetch problem url from cp-assist 🚫: {e}")


# 2️⃣ Get CSRF token and cookies from Safari (fresh)
def get_safari_csrf_and_cookie(url: str):
    """Uses AppleScript to get the CSRF token and PHPSESSID from Safari, then closes the tab."""
    script = f"""
    tell application "Safari"
        tell window 1
            set newTab to make new tab with properties {{URL:"{url}"}}
            set current tab to newTab
            delay 3
            set theCSRF to do JavaScript "document.querySelector('input[name=csrf_token]').value;" in newTab
            set theCookie to do JavaScript "document.cookie;" in newTab
            close newTab
        end tell
    end tell
    return theCSRF & ";;" & theCookie
    """

    result = subprocess.run(["osascript", "-e", script], capture_output=True, text=True)
    if result.returncode != 0:
        raise RuntimeError("AppleScript failed: " + result.stderr)
    csrf_token, cookie = result.stdout.strip().split(";;")
    return csrf_token, cookie


def fetch_testcases(url: str) -> list[dict[str, str]]:
    csrf, cookie = get_safari_csrf_and_cookie(url)
    headers = {
        "User-Agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7)",
        "Content-Type": "application/x-www-form-urlencoded",
        "Cookie": cookie,
        "Referer": url,
    }
    payload = {"csrf_token": csrf, "download": "true"}
    r = requests.post(url, headers=headers, data=payload)
    r.raise_for_status()

    if not r.content.startswith(b"PK\x03\x04"):
        raise RuntimeError("Server did not return a zip file")

    testcases: list[dict[str, str]] = []
    with zipfile.ZipFile(io.BytesIO(r.content)) as z:
        # collect i.in/i.out pairs in order
        ins = sorted([f for f in z.namelist() if f.endswith(".in")])
        outs = sorted([f for f in z.namelist() if f.endswith(".out")])
        for infile, outfile in zip(ins, outs):
            with z.open(infile) as f_in, z.open(outfile) as f_out:
                testcases.append(
                    {"input": f_in.read().decode(), "output": f_out.read().decode()}
                )
    return testcases


# 4️⃣ Send test cases to local cp-assist
def send_to_cp_assist(testcases):
    resp = requests.post("http://localhost:27121/test_cases", json=testcases)
    if resp.status_code == 200:
        print("Successfully added test cases ✅")
    else:
        print(f"Failed to add test cases: {resp.status_code}")


# Convert problem page URL to testcases URL
def convert_to_test_url(problem_url: str):
    if "/task/" not in problem_url:
        raise ValueError("Cannot convert problem URL to test URL")
    problem_id = problem_url.rstrip("/").split("/")[-1]
    return f"https://cses.fi/problemset/tests/{problem_id}/"


# ------------------ Main ------------------
if __name__ == "__main__":
    try:
        problem_url = fetch_problem_url()
    except Exception as e:
        print(e)
        exit(1)

    problem_url = fetch_problem_url()
    test_url = convert_to_test_url(problem_url)
    print(f"Fetching CSES problem test cases from {test_url}...")
    try:
        cases = fetch_testcases(test_url)
        print(f"Got {len(cases)} test cases 🧶")
    except Exception as e:
        print(f"Failed to fetch CSES test cases 😩: {e}")
        exit(1)

    send_to_cp_assist(cases)
