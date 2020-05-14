#!/usr/bin/env ruby
# frozen_string_literal: true

STDOUT.sync = true # DO NOT REMOVE
# Grab the pellets as fast as you can!

## A position
class Position
  attr_reader :x, :y

  def initialize(x, y) # rubocop:disable Naming/MethodParameterName
    @x = x
    @y = y
  end

  def to_s
    "#{@x} #{@y}"
  end

  def distance_to(other)
    dx = (@x - other.x).abs
    dy = (@y - other.y).abs

    c_squared = dx**2 + dy**2

    Math.sqrt(c_squared).round
  end

  def closest(others)
    closest = others[0]

    others.each do |pos|
      closest = pos if distance_to(pos) < distanceTo(closest)
    end
    closest
  end
end

## A Pellet with it's info
class Pellet
  attr_reader :pos, :value

  def initialize
    input = gets.chomp
    warn "Getting pellet position: #{input}"
    x, y, @value = input.split(' ').collect(&:to_i)
    @pos = Position.new x, y
  end
end

## A PacMan with all it's info
class PacMan
  attr_reader :pos, :pac_id,
              :mine, :type_id, :speed_turns_left, :ability_cooldown

  def initialize
    input = gets.chomp
    warn 'Pacman input:'
    warn input
    pac_id, mine, x, y, type_id, speed_turns_left, ability_cooldown = input.split(' ')
    @pac_id = pac_id.to_i # unique to player id
    @mine = mine.to_i == 1 # is this pac mine
    @pos = Position.new(x.to_i, y.to_i)
    @type_id = if type_id == 'ROCK'
                 :rock
               elsif type_id == 'PAPER'
                 :paper
               elsif type_id == 'SCISSORS'
                 :scissors
               end
    @speed_turns_left = speed_turns_left.to_i
    @ability_cooldown = ability_cooldown.to_i
  end
end

## Look Result info
class LookResult
  attr_reader :pos, :score, :distance, :enemies, :friends

  def initialize(opts = {
    score: 0,
    distance: 0,
    enemies: [],
    friends: []
  })
    @pos = opts[:pos]
    @score = opts[:score]
    @distance = opts[:distance]
    @enemies = opts[:enemies]
    @friends = opts[:friends]
  end
end

## The game board
class GameBoard
  def initialize
    # width: size of the grid
    # height: top left corner is (x=0, y=0)
    input = gets.chomp
    warn 'Initializing board: width + height'
    warn input
    @width, @height = input.split(' ').collect(&:to_i)
    @board = {}
    @height.times do |y|
      # one line of the grid: space " " is floor, pound "#" is wall
      row = gets.chomp
      warn "board line #{y}"
      warn row

      row.each_char.with_index do |val, x|
        pos = Position.new(x, y)
        @board[pos] =
          if val == '#'
            :wall
          else
            :floor
          end
      end
    end
  end

  def get_pos_info(pos)
    if @my_pacmen[pos]
      return :friendly
    elsif @other_pacmen[pos]
      return :enemy
    elsif @pellets[pos]
      return @pellets[pos]
    else
      return @board[pos]
    end
  end

  def update
    input = gets.chomp
    warn 'updating turn'
    warn input
    @my_score, @opponent_score = input.split(' ').collect(&:to_i)
    visible_pac_count = gets.to_i # all your pacs and enemy pacs in sight
    @my_pacmen = {}
    @other_pacmen = {}
    visible_pac_count.times do
      pacman = PacMan.new
      if pacman.mine
        @my_pacmen[pacman.pos] = pacman
      else
        @other_pacmen[pacman.pos] = pacman
      end
    end
    input = gets.chomp
    warn 'getting pellet count'
    warn input
    visible_pellet_count = input.to_i # all pellets in sight
    @pellets = {}
    visible_pellet_count.times do
      # value: amount of points this pellet is worth
      pellet = Pellet.new
      @pellets[pellet.pos] = pellet
    end
  end

  def up(pos, dist = 1)
    y =
      if (pos.y - dist).negative?
        @height - (dist - pos.y)
      else
        pos.y - dist
      end

    Position.new(pos.x, y)
  end

  def down(pos, dist = 1)
    y = if pos.y + dist > @height
          dist - pos.y
        else
          pos.y + dist
        end

    Position.new(pos.x, y)
  end

  def left(pos, dist = 1)
    x =
      if (pos.x - dist).negative?
        @width - (dist - pos.x)
      else
        pos.x - dist
      end

    Position.new(x, pos.y)
  end

  def right(pos, dist = 1)
    x = if pos.x + dist > @width
          dist - pos.x
        else
          pos.x + dist
        end
    Position.new(x, pos.y)
  end

  def look_up(pos)
    look(pos)
  end

  def look_down(pos)
    look(pos, -> { pos = down(pos) })
  end

  def look_left(pos)
    look(pos, -> { pos = left(pos) })
  end

  def look_right(pos)
    look(pos, -> { pos = right(pos) })
  end

  def look(pos, update_pos = lambda {
    pos = up(pos)
  })
    update_pos.call
    result = get_pos_info(pos)
    distance = 0
    score = 0
    friends = []
    enemies = []

    while result != :wall 
      warn "Found #{result.to_s} @ #{pos}"
      if result.class == Pellet
        score += result.value
      elsif result == :friendly
        friends.push @my_pacmen[pos]
      elsif result == :enemy
        enemies.push @other_pacmen[pos]
      end

      # update loop
      update_pos.call
      result = get_pos_info(pos)
      distance += 1
    end

    LookResult.new({
                    pos: pos, distance: distance, score: score, friends: friends, enemies: enemies
                   })
  end

  def command
    commands = []
    @my_pacmen.each do |pos, man|
      warn "#{man.pac_id} @ (#{pos.x}, #{pos.y})"
      up = look_up(pos)
      down = look_down(pos)
      right = look_right(pos)
      left = look_left(pos)

      # find the greatest score
      if up.score > down.score &&
        up.score > left.score &&
        up.score > right.score &&
        up.enemies.length == 0
        commands.push "MOVE #{man.pac_id} #{up.pos}"
      elsif down.score > up.score &&
        down.score > left.score &&
        down.score > right.score &&
        down.enemies.length == 0
        commands.push "MOVE #{man.pac_id} #{down.pos}"
      elsif right.score > left.score &&
        right.score > up.score &&
        right.score > down.score &&
        right.enemies.length == 0
        commands.push "MOVE #{man.pac_id} #{right.pos}"
      else
        commands.push "MOVE #{man.pac_id} #{left.pos}"
      end
    end

    puts commands.join(' | ')
  end
end

def main
  board = GameBoard.new

  # game loop
  loop do
    board.update
    # Write an action using puts
    # To debug: STDERR.puts "Debug messages..."

    board.command
  end
end

main
