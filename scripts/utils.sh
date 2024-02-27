# Function to print messages with a dynamic border, accounting for multi-line messages
print_message() {
  local msg="$1"
  local min_length="${2:-50}"
  local max_length=0

  # Split the message into lines and find the longest
  while IFS= read -r line; do
    ((${#line} > max_length)) && max_length=${#line}
  done <<<"$msg"

  # Ensure the message is at least `min_length` characters long
  if ((max_length < min_length)); then
    max_length=$min_length
  fi

  # Generate a border based on the length of the longest line
  local border=$(printf '%*s' "$max_length" '' | tr ' ' '-')

  echo "$border-"
  # Print each line of the message
  while IFS= read -r line; do
    printf "%s\n" "$line"
  done <<<"$msg"
  echo "$border-"
}
