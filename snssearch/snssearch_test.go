package snssearch_test

import (
	"testing"

	"github.com/hsmtkk/solid-invention/snssearch"
	"github.com/stretchr/testify/assert"
)

func Test0(t *testing.T) {
	tweets := []snssearch.Tweet{
		snssearch.NewTweet(1, "abcdefg", 2016, 9, 18),
		snssearch.NewTweet(2, "ABCDE", 2018, 9, 16),
		snssearch.NewTweet(3, "ahkldc", 2016, 9, 17),
		snssearch.NewTweet(4, "Gabd", 2016, 9, 18),
	}
	keywords := []string{"c", "ab"}
	searcher := snssearch.NewSearcher()
	got := searcher.Search(tweets, keywords)
	want := []int{3, 1, 4}
	assert.Equal(t, want, got)
}
