import os
import json
import requests

problem_id = 1
base_url = 'https://api.icfpcontest.com/problem?problem_id='
output_dir = 'problems/'

while True:
    url = base_url + str(problem_id)
    response = requests.get(url)

    # responseをjsonに変換します。
    try:
        data = response.json()
    except json.JSONDecodeError:
        # JSONDecodeErrorが起きた場合は、ループを終了します。
        print("Download finished or failed. Check the last problem_id: ", problem_id)
        break

    # "Success"の値が存在し、更にそれがJSONとして有効な場合にのみ保存を行います。
    if "Success" in data:
        try:
            data_to_save = json.loads(data["Success"])
        except json.JSONDecodeError:
            print(f"Failed to decode 'Success' value for problem_id={problem_id}. Skipping...")
            problem_id += 1
            continue
    else:
        print(f"'Success' key not found in response for problem_id={problem_id}. Stopping...")
        break

    # 出力するjsonファイルのパスを作成します。
    output_path = os.path.join(output_dir, f'problem-{problem_id}.json')

    # 既にファイルが存在している場合は、次の問題IDに移ります。
    if os.path.exists(output_path):
        print(f'File for problem_id={problem_id} already exists. Skipping...')
        problem_id += 1
        continue

    # JSONデータをファイルに書き込みます。
    with open(output_path, 'w') as outfile:
        json.dump(data_to_save, outfile)
        print(f'Successfully downloaded and wrote data for problem_id={problem_id}')

    problem_id += 1
