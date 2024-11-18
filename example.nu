# a simple example
{
  users: [
    {
      id: 1,
      username: "YuKun Liu",
      password: "123456",
      email: "mrxzx.info@gmail.com",
      update_at: (date now),
      permissions: [1001, 1002, 1003],
      configs: { admin: true, baned: false }
    }
  ]
} | sled-save db

# you can also save & load from tree
{
  user_log: [
    "2024-11-17: [id:1] Hello",
    "2024-11-18: [id:2] Hi"
  ]
} | sled-save db --tree logs
