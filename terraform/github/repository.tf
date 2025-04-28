resource "github_repository" "repository" {
  name        = local.repository
  description = "Scrape and extract metadata like title, description, images, and favicon from HTML documents."

  has_downloads        = false
  has_discussions      = false
  has_projects         = false
  has_wiki             = false
  has_issues           = true
  vulnerability_alerts = true
}
