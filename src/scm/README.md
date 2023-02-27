Module scm provides a unified interface to multiple source code management
systems including GitHub, GitHub Enterprise, Bitbucket, Bitbucket Server,
Gitee, Gitea and Gogs.

## Getting Started

Create a GitHub client:

```rust
use amp_common::scm;
use amp_common::scm::driver::github;

fn main() {
	let client = github::default();
}
```

Create a GitHub Enterprise client:

```rust
use amp_common::scm;
use amp_common::scm::driver::github;

fn main() {
	let client = github::new("https://github.company.com/api/v3");
}
```

Create a Bitbucket client:

```rust
use amp_common::scm;
use amp_common::scm::driver::bitbucket;

fn main() {
	let client = bitbucket::default();
}
```

Create a Bitbucket Server (Stash) client:

```rust
use amp_common::scm;
use amp_common::scm::driver::bitbucket;

fn main() {
	let client = bitbucket::new("https://stash.company.com");
}
```

Create a Gitea client:

```rust
use amp_common::scm;
use amp_common::scm::driver::gitea;

fn main() {
	let client = gitea::new("https://gitea.company.com");
}
```

Create a Gitee client:

```rust
use amp_common::scm;
use amp_common::scm::driver::gitee;

fn main() {
	let client = gitee::new("https://gitee.com/api/v5");
}
```

## Authentication

The scm client does not directly handle authentication. Instead, when creating
a new client, provide an `http.Client` that can handle authentication for you.
For convenience, this library includes oauth1 and oauth2 implementations that
can be used to authenticate requests.

```rust
use amp_common::scm;
use amp_common::scm::driver::github;
use amp_common::scm::transport;
use amp_common::scm::transport::oauth2;

fn main() {
	let mut client = github::default();

    // provide a custom http.Client with a transport
    // that injects the oauth2 token.
    client.client = http.Client{
        transport: oauth2.Transport{
        source: oauth2.StaticTokenSource(
            scm.Token{
            token: "ecf4c1f9869f59758e679ab54b4",
            },
        ),
        },
    };

    // provide a custom http.Client with a transport
    // that injects the private GitLab token through
    // the PRIVATE_TOKEN header variable.
    client.client = http.Client{
        transport: transport.PrivateToken{
        token: "ecf4c1f9869f59758e679ab54b4",
        },
    };

}
```

## Usage

The scm client exposes dozens of endpoints for working with repositories,
issues, comments, files and more.

Example code to get an issue:

```rust
let issue = client.issues.find("octocat/Hello-World", 1);
```

Example code to get a list of issues:

```rust
let opts = scm.IssueListOptions{
  page:   1,
  size:   30,
  open:   true,
  closed: false,
}

let issues = client.issues.list("octocat/Hello-World", opts);
```

Example code to create an issue comment:

```rust
let input = scm.CommentInput{
  body: "Found a bug",
}

let comment = client.issues.create_comment("octocat/Hello-World", 1, input);
```

## Useful links

Here are some useful links to providers API documentation:

- [Azure DevOps](https://docs.microsoft.com/en-us/rest/api/azure/devops/git/?view=azure-devops-rest-6.0)
- [Bitbucket cloud API](https://developer.atlassian.com/cloud/bitbucket/rest/intro/)
- [Bitbucket server/Stash API](https://docs.atlassian.com/bitbucket-server/rest/5.16.0/bitbucket-rest.html)
- [Gitea API](https://gitea.com/api/swagger/#/)
- [Gitee API](https://gitee.com/api/swagger/#/)
- [Github API](https://docs.github.com/en/rest/reference)
- [Gitlab API](https://docs.gitlab.com/ee/api/api_resources.html)
- [Gogs API](https://github.com/gogs/docs-api)

## Credits

Heavily inspired by [drone/go-scm](https://github.com/drone/go-scm)