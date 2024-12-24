package main

import (
	"fmt"
	"log"
	"os"
)

func main() {
	data, err := os.ReadFile("./input/2024/9.txt")
	if err != nil {
		log.Fatalf("failed to read file: %v", err)
	}

	blocks := make([]int, 0, len(data)/2)
	free := make([]int, 0, len(data)/2)
	for i, char := range data {
		num := int(char - '0')
		if i%2 == 0 {
			blocks = append(blocks, num)
		} else {
			free = append(free, -num)
		}
	}

	out, head, tail := make([]int, 0, len(data)/2), 0, len(blocks)-1
	for head <= tail {
		out = appendN(out, head, blocks[head])
		if head == tail {
			break
		}
		head++

		r := free[0] + blocks[tail]
		for {
			if r > 0 { // Not enough free space, append as much as possible
				out = appendN(out, tail, blocks[tail]-r)
				blocks[tail] = r
				break
			}

			// Enough space, append everything
			out = appendN(out, tail, blocks[tail])
			tail--

			if r == 0 { // No more free space
				break
			}

			r = blocks[tail] + r
		}
		free = free[1:]
	}

	total := 0
	for i, o := range out {
		total += o * i
	}

	fmt.Println(total)
}

func appendN(slice []int, val, n int) []int {
	for i := 0; i < n; i++ {
		slice = append(slice, val)
	}
	return slice
}
