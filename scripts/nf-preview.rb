#!/usr/bin/env ruby
require "yaml"

LANGUAGES_FILE = File.expand_path("../../languages.yaml", __FILE__)

languages = YAML.safe_load_file(ARGV[0] || LANGUAGES_FILE, symbolize_names: true)

languages.each do |language, attributes|
  icon = attributes[:icon]
  next if icon.nil?
  match = /\A\\u\{([A-F0-9]{4,})\}\z/i.match(icon)
  raise "Icon for #{language} is not in the correct format: `#{icon}`" unless match
  glyph = match.captures[0].hex.chr("UTF-8")
  puts "#{language}: #{glyph}"
end
