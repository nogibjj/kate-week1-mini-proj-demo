# Week3 Mini Proj Demo - Deploy Actix Random Fruit Microservice on AWS
I followed the [youtube example](https://www.youtube.com/watch?v=Im72N3or2FE) to build a random fruit microserive. After that, I use AWS ECR to make a container image and deploy it using AWS APP RUNNER. 
 You can access it here and have fun! https://5z6u5grkpf.us-east-1.awsapprunner.com/

## Page directory
* "/": The root simply displays the message "Hello World Random Fruit!"
* "/fruit": Prints a fruit randomly from a list of 10 fruits.
* "/health": Does a health check that returns status 200.
* "/version": Displays the version of cargo.

## References
*https://www.youtube.com/watch?v=Im72N3or2FE