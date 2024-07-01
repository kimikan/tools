package main

import (
	"archive/zip"
	"io"
	"os"
	"path/filepath"
	"strings"
)

func valid2(s string) bool {
	return strings.HasSuffix(s, ".proto") || strings.HasSuffix(s, ".pb.txt") || strings.HasSuffix(s, ".go") || strings.HasSuffix(s, ".cc") || strings.HasSuffix(s, ".h") || strings.HasSuffix(s, ".py") || strings.HasSuffix(s, ".sh") || strings.HasSuffix(s, ".c")
}

func valid(info os.FileInfo) bool {
	return info.Size() <= 3 * 1024 * 1024
}


func compress(src_dir string, zip_file_name string) {
	os.RemoveAll(zip_file_name)
	zipfile, _ := os.Create(zip_file_name)
	defer zipfile.Close()

	archive := zip.NewWriter(zipfile)
	defer archive.Close()

	filepath.Walk(src_dir, func(path string, info os.FileInfo, _ error) error {
		if path == src_dir {
			return nil
		}
		if info.IsDir() || true {
			if strings.Contains(path, ".bazel") || strings.Contains(path,".cache") || strings.Contains(path, "/third_party") {
		                return nil
		        }
		}
		if !info.IsDir() && !valid(info) {
			return nil
		}

		header, _ := zip.FileInfoHeader(info)
		header.Name = strings.TrimPrefix(path, src_dir+`/`)

		if info.IsDir() {
			header.Name += `/`
		} else {
			header.Method = zip.Deflate
		}
		writer, _ := archive.CreateHeader(header)
		if !info.IsDir() {
			file, _ := os.Open(path)
			defer file.Close()
			io.Copy(writer, file)
		}
		return nil
	})
}

func main() {
	compress("./", "test.so")
}
