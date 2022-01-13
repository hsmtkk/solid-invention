package pagination

type Problem struct {
	TotalItems   uint
	ItemsPerPage uint
}

func (p *Problem) PagesOfIndex(pageIndex uint) []uint {
	begin := (pageIndex-1)*p.ItemsPerPage + 1
	end := pageIndex*p.ItemsPerPage + 1
	pages := []uint{}
	for i := begin; i < end; i++ {
		pages = append(pages, i)
	}
	return pages
}
