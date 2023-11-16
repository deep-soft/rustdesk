#!/bin/bash

#file_in="sed-stuff.txt"
#file_in="${0%.*}".txt
#echo $file_in

if [[ "$1" != "" && -f "$1" ]]; then
  file_in="$1"
else
  file_in="${0%.*}".txt
fi

if [[ -f "$file_in" ]]; then
  echo "file_in:[$file_in]"
  cat $file_in | while read the_line
  do
    if [[ ${the_line:0:2} == "S:" ]]; then _str_=${the_line:2}; echo "str:[$_str_]" fi
    if [[ ${the_line:0:2} == "R:" ]]; then _rpl_=${the_line:2}; echo "rpl:[$_rpl_]" fi
    if [[ ${the_line:0:2} == "F:" ]]; then _fil_=${the_line:2}; echo "fil:[$_fil_]" fi
    if [[ "$_fil_" != "" ]]; then
      if [[ -f "$_fil_" ]]; then
        echo "sed -i \"s|$_str_|$_rpl_|\" $_fil_";
        sed -i "s|$_str_|$_rpl_|" $_fil_;
      else
        echo "not_found: [$_fil_]";
      fi
      _str_="";
      _rpl_="";
      _fil_="";
    fi
  done
fi
