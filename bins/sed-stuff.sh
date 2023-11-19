#!/bin/bash

debug_mode="N"

#file_in="sed-stuff.txt"
#file_in="${0%.*}".txt
#echo $file_in

echo_debug () {
if [[ $debug_mode == "Y" ]]; then
  echo "$1"
fi
}

if [[ "$1" != "" && -f "$1" ]]; then
  file_in="$1"
else
  file_in="${0%.*}".txt
fi

if [[ -f "$file_in" ]]; then
  count=0
  echo "file_in:[$file_in]"
  cat $file_in | while read -r the_line
  do
    ((count++)
    if [[ ${the_line:0:2} == "S:" ]]; then _str_=$(echo "${the_line:2}" | tr -d "\r"); echo_debug "str:[$_str_]"; fi
    if [[ ${the_line:0:2} == "R:" ]]; then _rpl_=$(echo "${the_line:2}" | tr -d "\r"); echo_debug "rpl:[$_rpl_]"; fi
    if [[ ${the_line:0:2} == "F:" ]]; then _fil_=$(echo "${the_line:2}" | tr -d "\r"); echo_debug "fil:[$_fil_]"; fi
    if [[ "$_fil_" != "" ]]; then
      if [[ -f "$_fil_" ]]; then
        echo_debug "sed -i \"s|$_str_|$_rpl_|\" $_fil_";
        sed -i "s|$_str_|$_rpl_|" $_fil_;
        _str_n_=$(grep "$_str_" "$_fil_");
        if [[ "$_str_n_" == "" ]]; then
          echo "OK_OK: $count";
        else
          echo "NOTOK: $count";
        fi
        if [[ $debug_mode == "Y" ]]; then
          echo "grep 1"
          grep "$_str_" "$_fil_";
          echo "grep 2"
          grep "_lastQueryTime" "$_fil_";
        fi
      else
        echo "not_found: [$_fil_]";
      fi
      _str_="";
      _rpl_="";
      _fil_="";
    fi
  done
  echo "count:[$count]"
fi
