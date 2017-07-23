package gigasecond

// import path for the time package from the standard library
import "time"

const testVersion = 4

func timeAdd(base time.Time, mod time.Time) time.Time {
	timeOut := time.Date(
		base.Year+mod.Year,
		base.month+mod.month,
		base.day+mod.day,
		base.hour+mod.hour,
		base.min+mod.min,
		base.sec+mod.sec,
		base.nsec,
		base.loc)
}

func AddGigasecond(timeIn time.Time) time.Time {
	giga := time.Date(31, 8, 6, 1, 46, 40, 0, time.UTC)

	time := timeAdd(timeIn, giga)

	return timeIn
}
