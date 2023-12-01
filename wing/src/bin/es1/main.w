bring cloud;
bring fs;

let findNumber = inflight (line: str, index: num): num? => {
    let numbers = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9",
    ];

    for wordIndex in 0..numbers.length {
        let word = numbers.at(wordIndex);

        if line.substring(index).startsWith(word) {
            let res = (wordIndex + 1) % 9;
            if res == 0 {
                return 9;
            }
            return res;
        }

    }

    return nil;
};

let calculateCalibration = inflight (line: str): num => {

    let line2 = line
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine");

    let var firstNumber: num? = nil;
    for index in 0..line.length {
        if let res = findNumber(line, index) {
            firstNumber = res;
            break;
        }
    }

    let var secondNumber: num? = nil;
    for index in line.length - 1..0 {
        if let res = findNumber(line, index) {
            secondNumber = res;
            break;
        }
    }

    if let first = firstNumber{
        if let second = secondNumber {
            return first * 10 + second;
        }

        return first * 10 + first;
    }

    return 0;
};

let sumCalibrations = inflight (input: str): str => {
    let lines = input.split("\n");
    let var result = 0;

    for line in lines {
        result += calculateCalibration(line);
    }

    return Json.stringify(result);
};

new cloud.OnDeploy(inflight () => {
    let var input = fs.readFile("in.txt");
    let var result = sumCalibrations(input);
    log("result is:" + result);
});


test "calculate calibration" {
	assert(calculateCalibration("one5") == 15);
	assert(calculateCalibration("2one5") == 25);
	assert(calculateCalibration("fiveight") == 58);
	assert(calculateCalibration("9nine") == 99);
	assert(calculateCalibration("75three1six67") == 77);
	assert(calculateCalibration("threethree3four64") == 34);
	assert(calculateCalibration("krpzm9vgzzcgjsxhnkckqv98lmjthbdqsixsix") == 96);
	assert(calculateCalibration("qcptwone1fiveqxfrtmbpdtdtfceightstfivenine") == 29);
	assert(calculateCalibration("two1nine") == 29);
	assert(calculateCalibration("eightwothree") == 83);
	assert(calculateCalibration("abcone2threexyz") == 13);
	assert(calculateCalibration("xtwone3four") == 24);
	assert(calculateCalibration("4nineeightseven2") == 42);
	assert(calculateCalibration("zoneight234") == 14);
	assert(calculateCalibration("7pqrstsixteen") == 76);
	assert(calculateCalibration("5gsjgpcrvq") == 55);
}