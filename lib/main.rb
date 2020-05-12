#!/usr/bin/env ruby
# frozen_string_literal: true

STDOUT.sync = true # DO NOT REMOVE
# Grab the pellets as fast as you can!

## A Pellet with it's info
class Pellet
  attr_reader :x, :y, :value

  def initialize
    @x, @y, @value = gets.split(' ').collect(&:to_i)
  end
end

## A PacMan with all it's info
class PacMan
  attr_reader :x, :y, :pac_id,
              :mine, :type_id, :speed_turns_left, :ability_cooldown

  def initialize
    pac_id, mine, x, y,
      type_id, speed_turns_left, ability_cooldown = gets.split(' ')
    @pac_id = pac_id.to_i # unique to player id
    @mine = mine.to_i == 1 # is this pac mine
    @x = x.to_i
    @y = y.to_i
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

## The game board
class GameBoard
  # change scope to public

  attr_reader :height, :width

  def initialize
    @height, @width = gets.split(' ').collect(&:to_i)
    @board = [[]]
    y = 0
    @height.times do
      # one line of the grid: space " " is floor, pound "#" is wall
      row = gets.chomp

      row.each_char.with_index do |val, x|
        @board[x][y] = if val == '#'
                         :wall
                       else
                         :floor
                       end
      end

      y += 1
    end
  end

  def update
    @my_score, @opponent_score = gets.split(' ').collect(&:to_i)
    visible_pac_count = gets.to_i # all your pacs and enemy pacs in sight
    @my_pacmen = []
    @other_pacmen = []
    visible_pac_count.times do
      pacman = PacMan.new
      if pacman.mine
        @my_pacmen.push pacman
      else
        @other_pacmen.push pacman
      end
    end
    visible_pellet_count = gets.to_i # all pellets in sight
    @pellets = []
    visible_pellet_count.times do
      # value: amount of points this pellet is worth
      @pellets.push(Pellet.new)
    end
  end
end

# width: size of the grid
# height: top left corner is (x=0, y=0)
board = GameBoard.new

# game loop
loop do
  board.update
  # Write an action using puts
  # To debug: STDERR.puts "Debug messages..."

  puts 'MOVE 0 15 10' # MOVE <pacId> <x> <y>
end
