# -*- coding: utf-8 -*-
import itertools, re, operator


FIRST_CAPITAL = re.compile('(.)([A-Z][a-z]+)')
ALL_CAPITAL = re.compile('([a-z0-9])([A-Z])')
SPLIT_ALPHA_DIGITS = re.compile(r'[^\d]+|\d+')


def camelcase2underscore(name):
    return ALL_CAPITAL.sub(r'\1_\2', FIRST_CAPITAL.sub(r'\1_\2', name)).lower()


def files_compare_key(file):
    key = [not hasattr(file, 'content')]
    for is_digit, group in itertools.groupby(SPLIT_ALPHA_DIGITS.findall(file.name),
    	                                     operator.methodcaller('isdigit')):
        if is_digit:
            for n in group:
                key.append(('', int(n)))
        else:
            key.append((''.join(group).lower(), 0))
    return key


def packages_compare_key(package):
	if package.name == 'default':
		return (0, None)
	if package.name == 'user':
		return (2, None)
	return (1, package.name)


