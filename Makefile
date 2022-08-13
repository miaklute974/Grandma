deploy:
	@echo "Deploying!"
	git checkout main
	git pull
	cargo build --release
	git push heroku main

logs:
	@echo "Printing prd logs!"
	heroku logs --tail