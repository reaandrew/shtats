#!/usr/bin/env python3

import subprocess
from dataclasses import dataclass
from subprocess import Popen, PIPE, STDOUT

import humanize as humanize


@dataclass
class GitObj:
    type: str
    hash: str
    size: int
    obj: str


@dataclass
class GitObjWrapper:
    size: int
    name: str


class GitObjs(dict):

    def add(self, obj: GitObj):
        if obj.obj in self:
            self[obj.obj].size += obj.size
            self[obj.obj].name = obj.obj
        else:
            self[obj.obj] = GitObjWrapper(
                size=obj.size,
                name=obj.obj
            )

    def size(self):
        return sum(x.size for x in self.values())


def main():
    objs = GitObjs()
    ls = subprocess.Popen(['git', 'rev-list', '--all', '--objects'], stdout=subprocess.PIPE)
    total = len(ls.stdout.readlines())
    ls = subprocess.Popen(['git', 'rev-list', '--all', '--objects'], stdout=subprocess.PIPE)
    count = 0
    for line in ls.stdout.readlines():
        count = count + 1
        print (f"{count} of {total} ({round(count/total * 100, 2)}%)", end="\r")
        p = Popen(['git', 'cat-file', "--batch-check=\"%(objecttype) %(objectname) %(objectsize) %(rest)\""],
                  stdout=PIPE, stdin=PIPE, stderr=PIPE)
        stdout_data = p.communicate(input=line)[0]
        line = stdout_data.decode().strip().strip('"')
        line_items = line.split(' ')
        obj = GitObj(
            type=line_items[0],
            hash=line_items[1],
            size=int(line_items[2]),
            obj=line_items[3])
        if obj.type == "blob":
            objs.add(obj)
    print(humanize.naturalsize(objs.size()*0.14))


if __name__ == '__main__':
    main()
