import os
import re

def clean_code_files(root_dir):
    # 需要清理的文件扩展名
    code_extensions = {'.rs', '.toml', '.go', '.py', '.json', '.yaml', '.yml'}
    # 正则匹配 Markdown 代码围栏
    fence_pattern = re.compile(r'^```[a-zA-Z]*\s*$', re.MULTILINE)

    for root, dirs, files in os.walk(root_dir):
        for file in files:
            file_path = os.path.join(root, file)
            _, ext = os.path.splitext(file)
            
            # 只处理代码文件，忽略 .md 文件
            if ext.lower() in code_extensions:
                print(f"正在清理: {file_path}")
                
                with open(file_path, 'r', encoding='utf-8') as f:
                    lines = f.readlines()
                
                # 过滤掉以 ``` 开头的行
                cleaned_lines = [line for line in lines if not fence_pattern.match(line.strip())]
                
                # 重新写入文件
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.writelines(cleaned_lines)

    print("代码清理完成，Markdown 围栏已全部移除。")

if __name__ == "__main__":
    # 当前目录即为 NGOS 项目根目录
    clean_code_files(".")