package main

import (
	"fmt"
	"math/rand"
	"sort"
	"testing"
	"time"
)

func Test_index(t *testing.T) {
	v := []string{"", "A", "B", "AA", "AB", "BA", "BB", "AAA", "AAB", "ABA", "ABB"}

	for i, s := range v {
		parent := parent(i)
		left, right := children(i)

		if left < len(v) {
			if v[left] != s+"A" {
				t.Errorf("Val: %v. Left: %v\n", s, v[left])
			}
		}
		if right < len(v) {
			if v[right] != s+"B" {
				t.Errorf("Val: %v. Right: %v\n", s, v[right])
			}
		}
		if i != 0 {
			if v[parent] != s[:len(s)-1] {
				t.Errorf("Val: %v. Parent: %v\n", s, v[parent])
			}
		}
	}
}

func Test_sift(t *testing.T) {
	h := Heap[int]{s: []int{}}
	comp := []int{}
	rand.Seed(time.Now().UnixNano())
	for i := 0; i < 100; i++ {
		a := rand.Intn(20)
		comp = append(comp, a)
		h.Push(a)
	}
	fmt.Printf("Starting comp:\n%v\n", comp)
	sort.Sort(sort.Reverse(sort.IntSlice(comp)))

	for _, v := range comp {
		b := h.Pop()
		if v != b {
			t.Errorf("\nWanted: %v. Got: %v\nHeap: %v\nComp: %v\n", v, b, h.s, comp)
		}
	}
}

func Test_this(t *testing.T) {
	v := []int{7, 2, 7, 7, 7, 3, 0, 1, 5, 5}

	h := Heap[int]{s: []int{}}
	comp := []int{}

	for _, val := range v {
		h.Push(val)
		comp = append(comp, val)
	}
	fmt.Printf("Starting heap:\n%v\n", h.s)
	fmt.Printf("Starting comp:\n%v\n", comp)

	sort.Sort(sort.Reverse(sort.IntSlice(comp)))
	for _, val := range comp {
		b := h.Pop()
		fmt.Printf("Heap:\n%v\n", h.s)

		if val != b {
			t.Errorf("\nWanted: %v. Got: %v\nHeap: %v\nComp: %v\n", val, b, h.s, comp)
		}
	}

}
