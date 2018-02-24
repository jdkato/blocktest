"""check_examples.py
"""
import filecmp
import os
import shutil
import subprocess


def check(src='examples', out='content'):
    """
    """
    shutil.move('target/debug/blocktest', os.getcwd())
    # TODO: Support more than one target extension at a time.
    subprocess.check_call(['./blocktest', 'examples', '.md', 'content'])

    for dir_name, _, files in os.walk(src):
        for fname in files:
            if fname.endswith('.md'):
                observed = os.path.join(dir_name, fname).replace(src, out)
                expected = os.path.join(dir_name, 'output.txt')
                assert filecmp.cmp(observed, expected), observed

if __name__ == '__main__':
    check()
