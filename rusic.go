package main

import (
	"database/sql"
	"fmt"
	_ "github.com/mattn/go-sqlite3"
	"math/rand"
	"os"
	"time"
)

func SayHello() string {
	return "Hello, World!"
}

func RandomArt() []string {
	db_path := os.Getenv("RUS_DB_PATH")
	db, err := sql.Open("sqlite3", db_path)
	if err != nil {
		fmt.Println("Error opening database: ", err)
	}
	defer db.Close()

	rows, err := db.Query("SELECT idx FROM music_images")
	if err != nil {
		fmt.Println("Error executing query: ", err)
	}
	defer rows.Close()

	idxlist := []int{}
	for rows.Next() {
		var idx int
		if err := rows.Scan(&idx); err != nil {
			fmt.Println("Error scanning row: ", err)
			continue
		}
		// fmt.Println("Index:", idx)
		idxlist = append(idxlist, idx)
	}

	// fmt.Printf("Index list: %v\n", idxlist)

	if err := rows.Err(); err != nil {
		fmt.Println("Error iterating over rows: ", err)
	}

	rand.Seed(time.Now().UnixNano())

	randomNumbers := []int{}
	for i := 0; i < 5; i++ {
		randomIndex := rand.Intn(len(idxlist))
		randomNumbers = append(randomNumbers, idxlist[randomIndex])
	}

	// fmt.Printf("Random numbers: %v\n", randomNumbers)

	thumbPaths := []string{}
	for _, idx := range randomNumbers {
		rows, err = db.Query(fmt.Sprintf("SELECT httpthumbpath FROM music_images WHERE idx=%d", idx))
		if err != nil {
			fmt.Println("Error executing query: ", err)
		}
		defer rows.Close()

		for rows.Next() {
			var thumbpath string
			if err := rows.Scan(&thumbpath); err != nil {
				fmt.Println("Error scanning row: ", err)
				continue
			}
			thumbPaths = append(thumbPaths, thumbpath)
		}
	}
	if err := rows.Err(); err != nil {
		fmt.Println("Error iterating over rows: ", err)
	}

	fmt.Printf("Thumb paths: %v\n", thumbPaths)

	return thumbPaths
}
