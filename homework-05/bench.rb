#!/usr/bin/env ruby

require 'open3'

loop do
  output, status = Open3.capture2e('./run.sh')

  failures = output.lines.grep(%r{Response is not valid 'application/json'.}).count

  success = output.include?('Result: {reduction/solutions=352.0}')

  time = output[/Execution took (\d+)ms./, 1]

  puts "#{success} | #{failures} | #{time} ms"
end
