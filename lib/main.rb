#!/usr/bin/env ruby
# frozen_string_literal: true

STDOUT.sync = true # DO NOT REMOVE
# Grab the pellets as fast as you can!

## An offset of a position
class Offset
  attr_reader :dist, :direction

  def initialize(dist, direction)
    @dist = dist
    @direction = direction
  end
end

## A position
class Position
  attr_reader :x, :y, :arena

  def initialize(x_pos, y_pos, arena)
    @x = x_pos
    @y = y_pos
    @arena = arena
  end

  def +(other)
    if other.class == Offset
      x = @x
      y = @y
      dist = other.dist
      case other.direction
      when :up
        y =
          (@y - dist).negative? ? @arena.height - (dist - @y) : @y - dist
      when :down
        y = @y + dist > @arena.height ? dist - @y : @y + dist
      when :right
        x = @x + dist > @arena.width ? dist - @x : @x + dist
      when :left
        x =
          (@x - dist).negative? ? @arena.width - (dist - @x) : @x - dist
      end

      Position.new(x, y, @arena)
    elsif other.class == Position
      x = @x + other.x
      y = @y + other.y
      Position.new(x, y, @arena)
    end
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

  def info
    if @arena.my_pacmen[self]
      :friendly
    elsif @arena.other_pacmen[self]
      :enemy
    elsif @arena.pellets[self]
      @arena.pellets[self]
    else
      @arena.board[self]
    end
  end

  def look_up
    look
  end

  def look_down
    look(-> { pos = down(pos) })
  end

  def look_left
    look(-> { pos = left(pos) })
  end

  def look_right
    look(-> { pos = right(pos) })
  end

  def look(update_pos = lambda {
    pos = up(pos)
  })
    update_pos.call
    result = info(pos)
    distance = 0
    score = 0
    friends = []
    enemies = []

    while result != :wall
      warn "Found #{result} @ #{pos}"
      if result.class == Pellet
        score += result.value
      elsif result == :friendly
        friends.push @arena.my_pacmen[pos]
      elsif result == :enemy
        enemies.push @arena.other_pacmen[pos]
      end

      # update loop
      update_pos.call
      result = info(pos)
      distance += 1
    end

    LookResult.new({
                     pos: pos, distance: distance, score: score, friends: friends, enemies: enemies
                   })
  end
end

## A Pellet with it's info
class Pellet
  attr_reader :pos, :value

  def initialize(arena)
    input = gets.chomp
    warn "Getting pellet position: #{input}"
    x, y, @value = input.split(' ').collect(&:to_i)
    @pos = Position.new x, y, arena
  end
end

## A PacMan with all it's info
class PacMan
  attr_reader :pos, :pac_id,
              :mine, :type_id, :speed_turns_left, :ability_cooldown

  def initialize(arena)
    input = gets.chomp
    warn 'Pacman input:'
    warn input
    pac_id, mine, x, y, type_id, speed_turns_left, ability_cooldown = input.split(' ')
    @pac_id = pac_id.to_i # unique to player id
    @mine = mine.to_i == 1 # is this pac mine
    @pos = Position.new(x.to_i, y.to_i, arena)
    @type_id =
      case type_id
      when 'ROCK'
        :rock
      when 'PAPER'
        :paper
      when 'SCISSORS'
        :scissors
      end
    @speed_turns_left = speed_turns_left.to_i
    @ability_cooldown = ability_cooldown.to_i
  end
end

## Look Result info
class LookResult
  attr_reader :pos, :score, :distance, :enemies, :friends

  def initialize(pos, offset)
    @pos = pos
    new_pos = pos + offset
    @distance = pos.distance_to new_pos
    info = pos.info
    @score = info.class == Pellet ? info.value : 0
  end
end

## The game board
class Arena
  attr_reader :my_pacmen, :other_pacmen, :pellets, :board, :width, :height

  def initialize
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
        pos = Position.new(x, y, self)
        @board[pos] =
          if val == '#'
            :wall
          else
            :floor
          end
      end
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
      pacman = PacMan.new(self)
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
      pellet = Pellet.new self
      @pellets[pellet.pos] = pellet
    end
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
         up.enemies.empty?
        commands.push "MOVE #{man.pac_id} #{up.pos}"
      elsif down.score > up.score &&
            down.score > left.score &&
            down.score > right.score &&
            down.enemies.empty?
        commands.push "MOVE #{man.pac_id} #{down.pos}"
      elsif right.score > left.score &&
            right.score > up.score &&
            right.score > down.score &&
            right.enemies.empty?
        commands.push "MOVE #{man.pac_id} #{right.pos}"
      else
        commands.push "MOVE #{man.pac_id} #{left.pos}"
      end
    end

    puts commands.join(' | ')
  end
end

def main
  board = Arena.new

  # game loop
  loop do
    board.update
    # Write an action using puts
    # To debug: STDERR.puts "Debug messages..."

    board.command
  end
end

main
