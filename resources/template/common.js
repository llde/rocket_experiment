function EVAPostRequest(address, response = null, error = null)
{
	this.address = address;
	this.onresponse = response;
	this.onerror = error;
	this.reqstr = "";
	
	this.addRequest = function(key, value)
	{
		if (this.reqstr != "")
			this.reqstr += "&";
		this.reqstr += encodeURIComponent(key) + '=' + encodeURIComponent(value);
	};
	
	this.send = function()
	{
		var xhr = new XMLHttpRequest();
		xhr.open('POST', address, true);
		xhr.setRequestHeader('Content-type', 'application/x-www-form-urlencoded');
		xhr.custom_onokresponse = this.onresponse;
		xhr.custom_onerrorresponse = this.onerror;
		xhr.onload = function()
		{
			if (this.readyState == 4 && this.status == 200)
			{
				if (this.responseText.substr(0, 1) == '0')
				{
					if (this.custom_onokresponse) this.custom_onokresponse();
				}
				else
				{
					if (this.custom_onerrorresponse) this.custom_onerrorresponse(this.responseText);
					else common_goTo('/template/error.html?value=' + this.responseText.substr(0, 4) + '&verb=' + encodeURIComponent(this.responseText.substr(4)));
				}
			}
			else
			{
				if (this.custom_onerrorresponse) this.custom_onerrorresponse(null);
				else common_goTo('/template/error.html');
			}
		};
		
		xhr.send(this.reqstr);
	};
}

function common_goTo(address)
{
	window.location.href = address;
}

function common_trim(s, mask)
{
    while (~mask.indexOf(s[0]))
	{
        s = s.slice(1);
    }
    while (~mask.indexOf(s[s.length - 1]))
	{
        s = s.slice(0, -1);
    }
    return s;
}