#[cfg(test)]
mod tests {
    #[test]
    fn readme_has_visual_overview_and_operator_sections() {
        let readme = include_str!("../README.md");
        for required in [
            "![Meridian RS architecture](docs/assets/meridian-rs-architecture.svg)",
            "## Operator quickstart",
            "## Command center",
            "## System map",
            "```mermaid",
            "## Readable logs",
            "MERIDIAN_LOG_STYLE=pretty",
        ] {
            assert!(readme.contains(required), "README missing: {required}");
        }
    }

    #[test]
    fn architecture_svg_has_labeled_runtime_components() {
        let svg = include_str!("../docs/assets/meridian-rs-architecture.svg");
        for required in [
            "Meridian RS",
            "Web UI",
            "CLI / REPL",
            "Screening",
            "Management",
            "Meteora DLMM",
            "Solana RPC",
        ] {
            assert!(svg.contains(required), "SVG missing: {required}");
        }
    }

    #[test]
    fn readme_roadmap_has_no_unfinished_checkboxes() {
        let readme = include_str!("../README.md");

        assert!(
            !readme.contains("- [ ]"),
            "README still has unchecked roadmap items"
        );
    }
}
