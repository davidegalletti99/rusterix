import subprocess
import os

def get_comment_prefixes(ext):
    if ext in ['.rs', '.ts', '.js', '.c', '.cpp', '.java', '.go']:
        return ['//']
    elif ext in ['.py', '.sh', '.toml', '.yml', '.yaml']:
        return ['#']
    else:
        return []

def count_loc():
    try:
        # Get list of files from git
        result = subprocess.run(['git', 'ls-files'], capture_output=True, text=True, check=True)
        files = result.stdout.splitlines()
    except subprocess.CalledProcessError:
        print("Error: Not a git repository or git not found.")
        return

    stats = {} # ext -> {'files': 0, 'code': 0, 'comments': 0, 'blanks': 0}
    
    total_files = 0
    total_code = 0
    total_comments = 0
    total_blanks = 0

    print(f"Total files found by git: {len(files)}")

    for file_path in files:
        # Exclude XML files
        if file_path.lower().endswith('.xml'):
            continue
        
        if os.path.isdir(file_path):
            continue

        ext = os.path.splitext(file_path)[1].lower()
        if not ext:
            ext = '(no extension)'
            
        if ext not in stats:
            stats[ext] = {'files': 0, 'code': 0, 'comments': 0, 'blanks': 0}

        comment_prefixes = get_comment_prefixes(ext)

        try:
            with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
                file_code = 0
                file_comments = 0
                file_blanks = 0
                
                for line in f:
                    stripped = line.strip()
                    if not stripped:
                        file_blanks += 1
                        continue
                    
                    is_comment = False
                    for prefix in comment_prefixes:
                        if stripped.startswith(prefix):
                            is_comment = True
                            break
                    
                    if is_comment:
                        file_comments += 1
                    else:
                        file_code += 1

                stats[ext]['files'] += 1
                stats[ext]['code'] += file_code
                stats[ext]['comments'] += file_comments
                stats[ext]['blanks'] += file_blanks
                
                total_files += 1
                total_code += file_code
                total_comments += file_comments
                total_blanks += file_blanks

        except Exception as e:
            print(f"Could not read {file_path}: {e}")

    print("\n=== CODE CONTENT ===")
    print(f"{'Extension':<15} {'Files':<10} {'Code Lines':<15}")
    print("-" * 40)
    for ext, data in sorted(stats.items(), key=lambda x: x[1]['code'], reverse=True):
        print(f"{ext:<15} {data['files']:<10} {data['code']:<15}")
    print("-" * 40)
    print(f"{'TOTAL':<15} {total_files:<10} {total_code:<15}")

    print("\n=== COMMENT CONTENT ===")
    print(f"{'Extension':<15} {'Files':<10} {'Comment Lines':<15}")
    print("-" * 40)
    for ext, data in sorted(stats.items(), key=lambda x: x[1]['comments'], reverse=True):
        print(f"{ext:<15} {data['files']:<10} {data['comments']:<15}")
    print("-" * 40)
    print(f"{'TOTAL':<15} {total_files:<10} {total_comments:<15}")

    print("\n=== SUMMARY ===")
    print(f"Total Files:    {total_files}")
    print(f"Total Code:     {total_code}")
    print(f"Total Comments: {total_comments}")
    print(f"Total Blanks:   {total_blanks}")


if __name__ == "__main__":
    count_loc()
