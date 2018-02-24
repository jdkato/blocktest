"""check_examples.py
"""
import filecmp
import os
import shutil
import subprocess


def check(src='examples', out='content', exts=['.md', '.jade']):
    """
    """
    shutil.move('target/debug/blocktest', os.getcwd())
    for ext in exts:
        subprocess.check_call(['./blocktest', 'examples', ext, 'content'])

    for dir_name, _, files in os.walk(src):
        for fname in files:
            if any(fname.endswith(ext) for ext in exts):
                observed = os.path.join(dir_name, fname).replace(src, out)
                expected = os.path.join(dir_name, 'output.txt')
                assert filecmp.cmp(observed, expected), observed

if __name__ == '__main__':
    check()
