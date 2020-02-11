tracker
=======

A small, Redis-backed URL request tracker application,
implementing 302 (temporary) redirects and bucketed metrics.

Purpose
-------

We want to understand the adoption of Genesis in the wild.
This tracker application is intended to provide us with that
information in an unobtrusive and non-invasive manner.  To that
end, it tracks when a kit version is first deployed to a BOSH
director, and we gather only that a deployment was made, of a
specific kit and version.

We track the following:

 - Kit Name (i.e. "cf" or "bosh")
 - Kit Version (i.e. "1.9.0")
 - Time of installation (to the nearest day)

We **DO NOT** track the following:

 - Genesis environment name
 - Environmental parameters
 - Enabled kit features
 - Secrets, certificates, or keys
 - IaaS configuration (what cloud, what region, etc.)
