package snssearch

import (
	"sort"
	"strings"
	"time"
)

type Tweet struct {
	ID        int
	Text      string
	Timestamp time.Time
}

func NewTweet(id int, text string, year, month, day int) Tweet {
	t := time.Date(year, time.Month(month), day, 0, 0, 0, 0, time.Local)
	return Tweet{ID: id, Text: text, Timestamp: t}
}

type Searcher struct{}

func NewSearcher() *Searcher {
	return &Searcher{}
}

func (s *Searcher) Search(tweets []Tweet, keywords []string) []int {
	matchingTweets := []Tweet{}
	for _, tw := range tweets {
		for _, kw := range keywords {
			if strings.Contains(tw.Text, kw) {
				matchingTweets = append(matchingTweets, tw)
				break
			}
		}
	}
	sort.Slice(matchingTweets, func(i, j int) bool {
		if matchingTweets[i].Timestamp.Equal(matchingTweets[j].Timestamp) {
			return matchingTweets[i].ID < matchingTweets[j].ID
		}
		return matchingTweets[i].Timestamp.Before(matchingTweets[j].Timestamp)
	})
	indexes := []int{}
	for _, tw := range matchingTweets {
		indexes = append(indexes, tw.ID)
	}
	return indexes
}
