package main

import (
	"golang.org/x/exp/constraints"
)

type Heap[T constraints.Ordered] struct {
	s []T
}

func (h *Heap[T]) Pop() T {
	res := h.s[0]
	h.s[0] = h.s[len(h.s)-1]
	h.s = h.s[:len(h.s)-1]
	h.sift_down(0)
	return res
}

func (h *Heap[T]) Push(el T) {
	h.s = append(h.s, el)
	h.sift_up(len(h.s) - 1)
}

func parent(i int) int {
	return (i - 1) / 2
}

func children(i int) (int, int) {
	return i*2 + 1, (i + 1) * 2
}

func (h *Heap[T]) sift_up(i int) {
	for {
		if i == 0 {
			return
		}
		r := parent(i)
		if h.s[i] <= h.s[r] {
			return
		}
		h.s[i], h.s[r] = h.s[r], h.s[i]
		i = r
	}
}

func (h *Heap[T]) sift_down(i int) {
	for {
		l, r := children(i)
		if len(h.s) == l+1 {
			if h.s[i] < h.s[l] {
				h.s[i], h.s[l] = h.s[l], h.s[i]
			}
			return
		}
		if len(h.s) <= l {
			return
		}
		a := 0
		if h.s[l] <= h.s[r] {
			a = r
		} else {
			a = l
		}
		if h.s[a] <= h.s[i] {
			return
		}
		h.s[i], h.s[a] = h.s[a], h.s[i]
		i = a
	}
}
