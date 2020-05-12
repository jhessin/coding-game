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
    x, y, @value = gets.split(' ').collect(&:to_i)
    @pos = Position.new x, y
  end
end

## A PacMan with all it's info
class PacMan
  attr_reader :pos, :pac_id,
              :mine, :type_id, :speed_turns_left, :ability_cooldown

  def initialize
    pac_id, mine, x, y,
      type_id, speed_turns_left, ability_cooldown = gets.split(' ')
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
  attr_reader :score, :distance, :enemies, :friends

  def initialize(opts = {
    score: 0,
    distance: 0,
    enemies: [],
    friends: []
  })
    @score = opts.score
    @distance = opts.distance
    @enemies = opts.enemies
    @friends = opts.friends
  end
end

## The game board
class GameBoard
  def initialize
    # width: size of the grid
    # height: top left corner is (x=0, y=0)
    @height, @width = gets.split(' ').collect(&:to_i)
    @board = {}
    y = 0
    @height.times do
      # one line of the grid: space " " is floor, pound "#" is wall
      row = gets.chomp

      row.each_char.with_index do |val, x|
        pos = Position.new(x, y)
        @board[pos] =
          if val == '#'
            :wall
          else
            :floor
          end
      end

      y += 1
    end
  end

  def get_pos_info(pos)
    if @my_pacmen[pos]
      :friendly
    elsif @other_pacmen[pos]
      :enemy
    elsif @pellets[pos]
      @pellets[pos]
    else
      @board[pos]
    end
  end

  def update
    @my_score, @opponent_score = gets.split(' ').collect(&:to_i)
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
    visible_pellet_count = gets.to_i # all pellets in sight
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
                     distance: distance, score: score, friends: friends, enemies: enemies
                   })
  end

  def command
    @my_pacmen.each do |man|
    end

    # Placeholder
    puts 'MOVE 0 15 10' # MOVE <pacId> <x> <y>
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
