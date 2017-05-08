package clock

const testVersion = 4

// Clock - Datatype representing a clock
type Clock struct {
	Hours   int
	Minutes int
}

func findHour(hour int) int {

	for hour >= 24 {
		hour /= 24
	}

	return hour
}

func findMinute(minuteIn int) (hourMod int, minute int) {
	hourMod = 0

	for minuteIn >= 60 {
		minuteIn -= 60
		hourMod += 1
	}

	return hourMod, minuteIn
}

func New(hour int, minute int) Clock {

	hourMod, minute := findMinute(minute)
	hour = findHour(hour + hourMod)

	clock := Clock{
		Hours:   hour,
		Minutes: minute,
	}

	return clock
}

func (c Clock) String() string {
	strHour := string(c.Hours)
	strMin := string(c.Minutes)

	if c.Hours < 10 {
		strHour = "0" + strHour
	}

	if c.Minutes < 10 {
		strMin = "0" + strMin
	}

	return strHour + ":" + strMin
}

func (c Clock) Add(minutes int) Clock {
	c.Minutes += minutes
	return c
}
