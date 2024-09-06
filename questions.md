* **What is the purpose of a tag?**
    [Reason to use tags](./nostr-whys/why_tags.md)

* **What is the purpose of a kind?**
    [Reasons to use kind](./nostr-whys/why_kinds.md)

* **What is the right kind number for DNS server?**
    A DNS domain can have multiple ip addresses. We only want the latest ip. The question is should we factor the case where a multipe ip of a same domain is alive at the same time? Based on these questions we have a choice to make. The kind has to be between `10000 <= n < 20000` or `30000 <= n < 40000`. Refer kinds section in [nips-01](https://github.com/nostr-protocol/nips/blob/master/01.md).

* **How can we use tags in our DNS server?**
    TODO

