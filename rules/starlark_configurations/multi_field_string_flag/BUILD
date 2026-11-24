load (":defs.bzl", "ice_cream", "flavor", "flavors", "flavor_flag")

flavor_flag(name = "flavor_flag", build_setting_default = "vanilla")

flavors(name = "flavors", flavors = [":grape", ":cherry", ":vanilla"])

flavor(name = "vanilla", color = "white")
flavor(name = "grape", color = "purple", type = "sugar-free")
flavor(name = "cherry", color = "red")

ice_cream(name = "ice_cream", flavors = ":flavors")
