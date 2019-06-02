#!/usr/bin/env python3

import re
import sys
import argparse

prototype_re = r'''
    (?P<return_type>\S+)
    \s+
    (?P<function_name>\S+)  
    \s*
    (?P<params>
        \(
        [^)]+
        \)
    )
    '''

extern_prototype_re=r'^extern\s+'+prototype_re+'\s*;'
extern_prototype = re.compile(extern_prototype_re, re.VERBOSE)

static_function_re = r'^static\s+(inline\s+)?'+prototype_re+'(?P<brace>\s*{)'
static_function = re.compile(static_function_re, re.VERBOSE)
generated_comment = '/* Generated patched file, do not edit */\n'
def external_function_names(header_path):
    function_names=set()
    with open(header_path) as header:
        for line in header:
            match = extern_prototype.search(line)
            if match:
                function_names.add(match.group('function_name'))
    return function_names

def replace_function_names(function_names, src_path, dst_path):
    def replace_function_name(match):
        line=match[0]
        function_name=match.group('function_name')
        if function_name in function_names:
            start_return=match.start('return_type')
            start=match.start('function_name')
            end=match.end('function_name')
            start_brace=match.start('brace')
            proto=line[start_return:start] + function_name + '_C' + line[end:start_brace]
            line='extern ' + proto + ';\n' + proto + line[start_brace:]
        return line

    with open(src_path) as src, open(dst_path, 'w') as dst:
        dst.write(generated_comment)
        for line in src:
            line=static_function.sub(replace_function_name, line)
            dst.write(line)


def patch_c(header_path, src_path, dst_path):
    function_names=external_function_names(header_path)
    replace_function_names(function_names, src_path, dst_path)

def main():
    parser = argparse.ArgumentParser(prog=sys.argv[0])
    parser.add_argument('--hdr', help='header file')
    parser.add_argument('--src', help='source C file')
    parser.add_argument('--dst', help='destination C file')
    args=parser.parse_args(sys.argv[1:])

    patch_c(args.hdr, args.src, args.dst)
if __name__ == "__main__":
    main()
