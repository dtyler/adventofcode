require 'digest'

class HashFinder
  def initialize(secret_key, front_padding_count)
    @secret_key = secret_key
    @front_padding_count = front_padding_count
    @Inf = 1.0 / 0.0
  end

  def compute
    padding = '0' * @front_padding_count
    (1..@Inf).find do |suffix|
      Digest::MD5.hexdigest(@secret_key + suffix.to_s).start_with?(padding)
    end
  end
end
