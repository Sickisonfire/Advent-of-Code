// input file: input.txt


const fs = require('fs');
let nums
let result
fs.readFile('input.txt', (err, data) => {
	if (err) throw err
	
	nums = data.toString().split('\n').map((el)=> parseInt(el))
	//console.log(nums)
	loop1:
	for (let i =0; i < nums.length; i++) {
		for (let j = i+1; j <= nums.length; j++) {
			//Part 1
			//if (nums[i] + nums[j] === 2020) {
			//	result = nums[i] * nums[j]
			//	console.log(result,nums[i], nums[j])
			//	break loop1
			//}
			//Part 2
			for (let k = j+1; k <= nums.length; k++) {
				if (nums[i] + nums[j] + nums[k] === 2020) {
					result = nums[i] * nums[j] * nums[k]
					console.log(result,nums[i], nums[j], nums[k])
					break loop1
				}

			}
		}

	}
})

