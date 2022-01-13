package isvalidip

import "net"

func IsValidIP(s string) bool {
	ip := net.ParseIP(s)
	return ip != nil
}
