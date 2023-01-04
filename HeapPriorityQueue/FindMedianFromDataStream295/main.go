package main

import "fmt"

type MedianFinder struct {
	l Heap[int]
	r Heap[int]
}

func Constructor() MedianFinder {
	return MedianFinder{
		l: Heap[int]{},
		r: Heap[int]{},
	}
}

func (this *MedianFinder) AddNum(num int) {
	if len(this.l.s) == 0 {
		this.l.Push(num)
		return
	}
	if float64(num) <= this.FindMedian() {
		this.l.Push(num)
	} else {
		this.r.Push(-num)
	}

	for len(this.l.s)+1 < len(this.r.s) {
		this.l.Push(-this.r.Pop())
	}
	for len(this.r.s)+1 < len(this.l.s) {
		this.r.Push(-this.l.Pop())
	}
}

func (this *MedianFinder) FindMedian() float64 {
	if len(this.l.s) == len(this.r.s) {
		return float64(this.l.s[0]-this.r.s[0]) / 2
	} else if len(this.l.s) < len(this.r.s) {
		return float64(-this.r.s[0])
	}
	return float64(this.l.s[0])
}

func main() {
	s := Constructor()
	s.AddNum(1)
	s.AddNum(2)
	fmt.Printf("Median: %v\n", s.FindMedian())
	s.AddNum(1)
	fmt.Printf("Median: %v\n", s.FindMedian())
}
