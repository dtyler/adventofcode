require 'digest'

class HashFinder
  def initialize(secret_key)
    @secret_key = secret_key
  end

  def compute
    candidate = 0
    hash = Digest::MD5.hexdigest("#{@secret_key}#{candidate}")
    until hash[0..4].eql? '00000'
      candidate += 1
      hash = Digest::MD5.hexdigest("#{@secret_key}#{candidate}")
    end
    candidate
  end
end
