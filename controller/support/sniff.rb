#! /usr/bin/env ruby
require 'colorb'

HEADER = {
	# version
	0 ... 2 => -> s {
		s.color(255).standout
	},

	# status
	2 ... 4 => -> s {
		s.color(255).underline
	},

	# sequence
	4 ... 8 => -> s {
		s.color(255)
	},
}

IDLE = {
	8 ... 12 => -> s {
		s.color(233)
	},

	16 ... 64 => -> s {
		s.color(233)
	}
}

INPUT = {
	# buttons
	8 ... 11 => -> s {
		s.color(3)
	},

	# left trigger
	11 ... 12 => -> s {
		s.color(63, 255).underline.standout
	},

	# right trigger
	12 ... 13 => -> s {
		s.color(63, 255).standout
	},

	# padding
	13 ... 16 => -> s {
		s.color(233)
	},

	# left pad x
	16 ... 18 => -> s {
		s.color(27, 255).underline.standout
	},

	# left pad y
	18 ... 20 => -> s {
		s.color(27).underline
	},

	# right pad x
	20 ... 22 => -> s {
		s.color(36, 255).standout
	},

	# right pad y
	22 ... 24 => -> s {
		s.color(36)
	},

	# left trigger precise
	24 ... 26 => -> s {
		s.color(63).underline
	},

	# right trigger precise
	26 ... 28 => -> s {
		s.color(63)
	},

	# orientation roll
	28 ... 30 => -> s {
		s.color(124)
	},

	# orientation yaw
	30 ... 32 => -> s {
		s.color(160)
	},

	# orientation pitch
	32 ... 34 => -> s {
		s.color(196)
	},

	# orientation roll
	34 ... 36 => -> s {
		s.color(57)
	},

	# orientation yaw
	36 ... 38 => -> s {
		s.color(93)
	},

	# orientation pitch
	38 ... 40 => -> s {
		s.color(129)
	},

	# garbage?
	48 ... 64 => -> s {
		s.color(233)
	},
}

BUS = ARGV.shift

unless BUS
  $stderr.puts "Usage: #$0 <bus>"
	exit 1
end

SESSION = IO.popen(%Q{tshark -l -i usbmon#{BUS} -T fields -e usb.capdata -Y 'usb.dst == "host"'}, err: '/dev/null')

SESSION.each_line {|line|
	line.strip!
	next if line.empty?
	bytes  = line.split(/\s*:\s*/)
	header = bytes[0 ... 8]
	body   = bytes[8 .. -1]

	header.each_with_index {|byte, n|
		if color = HEADER.find { |(r, _)| r === n }
			print color.last.(byte)
		else
			print byte.color(237)
		end

		if (n + 1) % 4 == 0
			print ' '
		end
	}

	# idle
	if header[2] == '04' && header[3] == '0b'
		body.each_with_index {|byte, n|
			n += 8

			if color = IDLE.find { |(r, _)| r === n }
				print color.last.(byte)
			else
				print byte.color(237)
			end

			if (n + 1) % 4 == 0
				print ' '
			end
		}
	end

	# input
	if header[2] == '01' && header[3] == '3c'
		body.each_with_index {|byte, n|
			n += 8

			if color = INPUT.find { |(r, _)| r === n }
				print color.last.(byte)
			else
				print byte.color(237)
			end

			if (n + 1) % 4 == 0
				print ' '
			end
		}
	end

	puts
}
