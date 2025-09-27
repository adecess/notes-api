###################################################################################################
# Dev
###################################################################################################
.PHONY: dev
dev:
	@echo "Starting development environment..."
	sudo docker-compose up --build

.PHONY: dev_detached
dev_detached:
	@echo "Starting development environment in background..."
	sudo docker-compose up --build -d

.PHONY: dev_logs
dev_logs:
	@echo "Showing development logs..."
	sudo docker-compose logs -f

.PHONY: dev_stop
dev_stop:
	@echo "Stopping development environment..."
	sudo docker-compose down

.PHONY: test
test:
	@echo "Running tests..."
	cargo test

.PHONY: check
check:
	@echo "Running cargo check..."
	cargo check

.PHONY: update_dependencies
update_dependencies:
	@echo "Updating Rust dependencies..."
	cargo update
	@echo "Dependencies updated successfully!"

.PHONY: audit_dependencies
audit_dependencies:
	@echo "Auditing Rust dependencies for security vulnerabilities..."
	cargo audit
	@echo "Dependency audit completed!"

.PHONY: clean
clean:
	@echo "Cleaning build artifacts..."
	cargo clean
	sudo docker-compose down --volumes --remove-orphans
	sudo docker system prune -f
	@echo "Clean completed!"

.PHONY: db_migrate
db_migrate:
	@echo "Running database migrations..."
	cargo run --bin notes_main -- migrate

###################################################################################################
# Build
###################################################################################################
.PHONY: build
build:
	@echo "Building Rust application..."
	cargo build --release
	@echo "Build completed!"

.PHONY: docker_build
docker_build:
	@echo "Building Docker image..."
	sudo docker build -f Dockerfile.notes-server -t notes-api:latest .
	@echo "Docker build completed!"

###################################################################################################
# Utilities
###################################################################################################
.PHONY: help
help:
	@echo "Available commands:"
	@echo ""
	@echo "Development:"
	@echo "  dev              - Start development environment"
	@echo "  dev_detached     - Start development environment in background"
	@echo "  dev_logs         - Show development logs"
	@echo "  dev_stop         - Stop development environment"
	@echo "  test             - Run tests"
	@echo "  check            - Run cargo check"
	@echo ""
	@echo "Dependencies:"
	@echo "  update_dependencies - Update Rust dependencies"
	@echo "  audit_dependencies  - Audit dependencies for vulnerabilities"
	@echo "  clean            - Clean build artifacts and containers"
	@echo ""
	@echo "Build:"
	@echo "  build            - Build Rust application"
	@echo "  docker_build     - Build Docker image"
	@echo ""
	@echo "Database:"
	@echo "  db_migrate       - Run database migrations"
	@echo ""
	@echo "Utilities:"
	@echo "  help             - Show this help message"


