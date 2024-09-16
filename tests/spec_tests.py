#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import sys
import argparse
import re
import json
import pathlib

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='Run cmark tests.')
    parser.add_argument('-s', '--spec', dest='spec', nargs='?', default='spec.txt',
            help='path to spec')
    parser.add_argument('-P', '--pattern', dest='pattern', nargs='?',
            default=None, help='limit to sections matching regex pattern')
    parser.add_argument('-d', '--dump-tests', dest='dump_tests',
            action='store_const', const=True, default=False,
            help='dump tests in JSON format')
    parser.add_argument('--debug-normalization', dest='debug_normalization',
            action='store_const', const=True,
            default=False, help='filter stdin through normalizer for testing')
    parser.add_argument('-n', '--number', type=int, default=None,
            help='only consider the test with the given number')
    args = parser.parse_args(sys.argv[1:])

def out(str):
    sys.stdout.buffer.write(str.encode('utf-8'))

def print_test_header(headertext, example_number, start_line, end_line):
    out("Example %d (lines %d-%d) %s\n" % (example_number,start_line,end_line,headertext))

def get_tests(specfile):
    filename = pathlib.Path(specfile).stem
    line_number = 0
    start_line = 0
    end_line = 0
    example_number = 0
    markdown_lines = []
    html_lines = []
    state = 0  # 0 regular text, 1 markdown example, 2 html output
    extensions = []
    headertext = ''
    tests = []

    header_re = re.compile('#+ ')

    with open(specfile, 'r', encoding='utf-8', newline='\n') as specf:
        for line in specf:
            line_number = line_number + 1
            l = line.strip()
            if l.startswith("`" * 32 + " example") or l.startswith("`" * 32 + ".example"):
                state = 1
                extensions = l[32 + len(" example"):].split()
            elif l == "`" * 32:
                state = 0
                example_number = example_number + 1
                end_line = line_number
                tests.append({
                    "markdown":''.join(markdown_lines).replace('→',"\t"),
                    "html":''.join(html_lines).replace('→',"\t"),
                    "example": example_number,
                    "start_line": start_line,
                    "end_line": end_line,
                    "section": (headertext or filename).replace('-', '_'),
                    "extensions": extensions})
                start_line = 0
                markdown_lines = []
                html_lines = []
            elif l == ".":
                state = 2
            elif state == 1:
                if start_line == 0:
                    start_line = line_number - 1
                markdown_lines.append(line)
            elif state == 2:
                html_lines.append(line)
            elif state == 0 and re.match(header_re, line):
                headertext = header_re.sub('', line).strip()
    return tests

if __name__ == "__main__":
    if args.debug_normalization:
        out(sys.stdin.read())
        exit(0)

    all_tests = get_tests(args.spec)
    if args.pattern:
        pattern_re = re.compile(args.pattern, re.IGNORECASE)
    else:
        pattern_re = re.compile('(.)|(^$)')
    tests = [
        test for test in all_tests
        if re.search(pattern_re, test['section'])
        and (not args.number or test['example'] == args.number)
    ]

    if args.dump_tests:
        out(json.dumps(tests, indent=2))
        exit(0)
    else:
        parser.print_help()
