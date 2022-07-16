# Auto-generated code below aims at helping you parse
# the standard input according to the problem statement.

read -r N
s=()
for (( i=0; i<$N; i++ )); do
    read -r p
    s+=($p)
done

echo ${s[*]} >&2


IFS=$'\n' sorted=($(sort -n <<<"${s[*]}"))
unset IFS

echo ${sorted[*]} >&2

diff=()
for (( i=1; i<=${#sorted[@]}-1; i++ )); do
    diff+=( $((${sorted[i]}-${sorted[i-1]})) )
done

echo ${diff[*]} >&2

min=${diff[0]}

for i in "${diff[@]}"; do
  (( i < min )) && min=$i
done

echo $min
