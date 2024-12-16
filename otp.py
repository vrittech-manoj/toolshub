import pyotp
import time

# Shared Secret Key
shared_key = "kasajsdkljlkjoiuios"

# Create TOTP Object
totp = pyotp.TOTP(shared_key)

# Generate OTP
otp = totp.now()
print(f"Generated OTP: {otp}")

# Verify OTP (Valid for 30 seconds by default)
user_input_otp = otp  # Replace with user input
is_valid = totp.verify(user_input_otp)
print(f"Is OTP valid? {is_valid}")
