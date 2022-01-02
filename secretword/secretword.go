package secretword

import "strings"

type Board struct {
	board [][]string
}

func NewBoard(s string) Board {
	board := [][]string{}
	for _, line := range strings.Split(s, "\n") {
		chs := []string{}
		for _, ch := range line {
			chs = append(chs, string(ch))
		}
		board = append(board, chs)
	}
	return Board{board}
}

func (b *Board) Find(word string) Location {
	for row := 0; row < len(b.board); row++ {
		for column := 0; column < len(b.board[0]); column++ {
			for i := 0; i < len(b.board); i++ {
				w := b.getWord(row, column, i)
				if word == w {
					return NewLocation(column, row)
				}
			}
		}
	}
	return Location{}
}

func (b *Board) getWord(row, column, length int) string {
	chs := []string{}
	for i := 0; i < length; i++ {
		if row+i >= len(b.board) {
			break
		}
		if column+i >= len(b.board[0]) {
			break
		}
		chs = append(chs, b.board[row+i][column+i])
	}
	return strings.Join(chs, "")
}

type Location struct {
	column int
	row    int
}

func NewLocation(column, row int) Location {
	return Location{column, row}
}
