require 'open-uri'
require 'json'
require 'yaml'

# colors.json URL
colors_json_url  = "https://raw.githubusercontent.com/github/personal-website/master/_data/colors.json"
# languages.yml URL
language_yml_url = "https://raw.githubusercontent.com/github/linguist/master/lib/linguist/languages.yml"

# Hash of colors.json
colors_hash = JSON.parse(open(colors_json_url, &:read))

# Hash of languages.yml
languages_hash = YAML.load(open(language_yml_url, &:read))


colors_hash.each{|language, value|
  # Skip if nil
  if languages_hash[language].nil?
    puts("[WRRN] languages_hash[language] is nill where language == #{language}")
    next
  end
  # Get color from github/linguist
  gh_color = languages_hash[language]["color"]

  # Get current color
  curr_color = value["color"]
  if curr_color != gh_color
    puts("[INFO] Language #{language}'s color changed: #{curr_color} => #{gh_color}'")
    # Update the color
    value["color"] = gh_color
  end
}

# Save new colors.json in pwd
new_colors_json = JSON.pretty_generate(colors_hash, :indent => "    ")
File.write("colors.json", new_colors_json)
puts("colors.json generated!")
