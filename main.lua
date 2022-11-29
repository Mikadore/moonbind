moonbind = require('moonbind')

print('Getting data: ')
data = moonbind.get_data({
    data = {1,2,3,4}
})
print(data)
for k, v in pairs(data) do
    print(string.format("Key '%s' has type '%s'", k, type(v)))
end
-- Steal cookie >:)
data.maybe = nil
print('Got data: ')
moonbind.print_data(data)