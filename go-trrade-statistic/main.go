package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"strconv"
	"strings"
)

func readConfig(fileName string) []string {
	lines := []string{}

	file, err := os.Open(fileName)
	if err == nil {
		defer file.Close()

		reader := bufio.NewReader(file)

		for {
			line, _, err := reader.ReadLine()

			if err == io.EOF {
				break
			}

			if err == nil {
				lines = append(lines, string(line))
			}
		}
	}

	return lines
}

//Item defines
type Item struct {
	assets      float64
	marketValue float64
}

func updateResult(user, assets, marketvalue string, results map[string]*Item) {
	assetsInt, err := strconv.ParseFloat(assets, 0)
	if err != nil {
		return
	}

	marketValueInt, err2 := strconv.ParseFloat(marketvalue, 0)
	if err2 != nil {
		return
	}

	v, ok := results[user]
	if !ok {
		v = &Item{}
		results[user] = v
	}

	if v.assets < assetsInt {
		v.assets = assetsInt
	}

	if v.marketValue < marketValueInt {
		v.marketValue = marketValueInt
	}
}

func filter(fileName string, results map[string]*Item) {

	file, err := os.Open(fileName)
	if err == nil {
		defer file.Close()

		reader := bufio.NewReader(file)
		user := ""
		assets := ""
		marketValue := ""

		for {
			line, _, err := reader.ReadLine()

			if err == io.EOF {
				break
			}

			linestr := string(line)

			if strings.Contains(linestr, "query/fund:") {
				index := strings.Index(linestr, "query/fund:")
				if index >= 0 {
					user = linestr[index+11:]
				}
			} else if strings.Contains(linestr, "money_type=") {
				if len(user) > 0 {
					index := strings.Index(linestr, "market_value=")
					if index >= 0 {
						marketValue = linestr[index+13:]
						index = strings.Index(marketValue, ",")
						if index > 0 {
							marketValue = marketValue[:index]
						}
					}

					index = strings.Index(linestr, "asset_balance=")
					if index >= 0 {
						assets = linestr[index+14:]
						index = strings.Index(assets, ",")
						if index > 0 {
							assets = assets[:index]
						}

						if len(user) > 0 {
							updateResult(user, assets, marketValue, results)
							user = ""
						}
					}
				}
			} //end if?
		}
	}
}

func main() {
	results := make(map[string]*Item)

	for _, line := range readConfig("configuration") {

		fmt.Println(line)
		filter(line, results)
	}

	for key, value := range results {
		fmt.Println(key, value)
	}

	//fmt.Println(results)
}
