# Changelog

All notable changes to `edge-domain` are documented here.

## [Unreleased]

### Added
- New `base` feature (`dep:edge-application-base`), pulled in transitively by `handler` and
  `service`; re-exports `Request`/`Response` (issue #139).

### Changed
- `Domain::echo_handler`/`Domain::new_handler_registry`/`Domain::new_service_registry`'s generic
  bounds now require `edge_application_base::Request`/`Response` instead of bare
  `Send + 'static`, matching `domain-handler`/`domain-service`'s breaking change (issue #139).
- arch audit rules 208, 112, 218, 120: fix layout; remove gateway/
- arch audit rule 200: implement event_type via self.kind in stage event types
- arch audit rules 39, 42: add README.md and CHANGELOG.md
- arch audit rules 221, 222: add _happy/_error/_edge test coverage for svc and trait functions
- arch audit rule 216: align test file names with source module names

## [0.8.0]

- Initial public release of L2 domain contracts
