run:
	@echo "Running dev server!"
	cargo run src/main.rs

deploy:
	@echo "Deploying!"
	git checkout main
	git pull
	cargo build --release
	git push heroku main

logs:
	@echo "Printing prd logs!"
	heroku logs --tail