
package main

import (
	"encoding/json"
	"log"
	"net/http"
	"time"
)

type Health struct {
	Service string `json:"service"`
	Status  string `json:"status"`
	TimeUTC string `json:"time_utc"`
}

func main() {
	mux := http.NewServeMux()
	mux.HandleFunc("/healthz", func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		_ = json.NewEncoder(w).Encode(Health{
			Service: "ngos-control-plane",
			Status:  "ok",
			TimeUTC: time.Now().UTC().Format(time.RFC3339),
		})
	})
	log.Println("ngos-control-plane listening on :8080")
	log.Fatal(http.ListenAndServe(":8080", mux))
}

